// Copyright 2015, 2016 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

import React, { Component, PropTypes } from 'react';
import { connect } from 'react-redux';
import { bindActionCreators } from 'redux';
import { observer } from 'mobx-react';

import UpgradeStore from '~/modals/UpgradeParity/store';

import Connection from '../Connection';
import ParityBar from '../ParityBar';

import Snackbar from './Snackbar';
import Container from './Container';
import DappContainer from './DappContainer';
import FrameError from './FrameError';
import Status from './Status';
import Store from './store';
import TabBar from './TabBar';

import styles from './application.css';

const inFrame = window.parent !== window && window.parent.frames.length !== 0;

@observer
class Application extends Component {
  static contextTypes = {
    api: PropTypes.object.isRequired,
    background: PropTypes.string
  }

  static propTypes = {
    blockNumber: PropTypes.object,
    children: PropTypes.node,
    isTest: PropTypes.bool,
    netChain: PropTypes.string,
    pending: PropTypes.array
  }

  store = new Store(this.context.api);
  upgradeStore = new UpgradeStore(this.context.api);

  render () {
    const [root] = (window.location.hash || '').replace('#/', '').split('/');
    const isDapp = root === 'app';

    if (inFrame) {
      return (
        <FrameError />
      );
    }

    return (
      <div>
        { isDapp ? this.renderDapp() : this.renderApp() }
        <Connection />
        <ParityBar dapp={ isDapp } />
      </div>
    );
  }

  renderApp () {
    const { blockNumber, children, pending, netChain, isTest } = this.props;

    return (
      <Container
        upgradeStore={ this.upgradeStore }
        onCloseFirstRun={ this.store.closeFirstrun }
        showFirstRun={ this.store.firstrunVisible }>
        <TabBar
          netChain={ netChain }
          isTest={ isTest }
          pending={ pending } />
        <div className={ styles.content }>
          { children }
        </div>
        {
          blockNumber
            ? <Status upgradeStore={ this.upgradeStore } />
            : null
        }
        <Snackbar />
      </Container>
    );
  }

  renderDapp () {
    const { children } = this.props;

    return (
      <DappContainer>
        { children }
      </DappContainer>
    );
  }
}

function mapStateToProps (state) {
  const { blockNumber, netChain, isTest } = state.nodeStatus;
  const { hasAccounts } = state.personal;
  const { pending } = state.signer;

  return {
    blockNumber,
    hasAccounts,
    isTest,
    netChain,
    pending
  };
}

function mapDispatchToProps (dispatch) {
  return bindActionCreators({}, dispatch);
}

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(Application);