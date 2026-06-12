//! Guild Artisan — `{1}{R}` Legendary Enchantment — Background.
//! "Commander creatures you own have 'Whenever this creature attacks
//! a player, if no opponent has more life than that player, you
//! create two Treasure tokens.'"
//!
//! Implementation: an ETB trigger installs a
//! `ContinuousEffect::filtered_grant_triggered` (Background recipe)
//! granting commander creatures a `SelfAttacks` triggered ability
//! that mints two Treasures via `Effect::CreateCommodityToken`.
//! "You own" is approximated by `controlled_by(ControllerConstraint::
//! You)`. The "attacks a PLAYER" restriction and the "no opponent has
//! more life than that player" gate both depend on the attacked
//! player, which an intervening-if fn cannot see — following the
//! catalog's defender-check convention (cf. Frontier Warmonger), the
//! effect fn inspects `trig.trigger_event`'s `DefendingEntity` and
//! returns no effects when the defender is not a player or a richer
//! opponent exists (a resolution-time check rather than a CR 603.4
//! stack-add gate; never over-fires the effect).

use arcana_core::combat::DefendingEntity;
use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::events::GameEvent;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
    GRANTED_TRIGGER_ID_BASE,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Guild Artisan");
    let background = reg.interner_mut().intern("Background");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(background);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install_grant,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_install_grant(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_grant_triggered(
            trig.source,
            ObjectFilter::creature()
                .commander_only()
                .controlled_by(ControllerConstraint::You),
            TriggeredAbilityDef {
                id: GRANTED_TRIGGER_ID_BASE + 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: granted_attack_treasures,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

/// Granted ability: "Whenever this creature attacks a player, if no
/// opponent has more life than that player, you create two Treasure
/// tokens." The defender/life gate is checked against the firing
/// event; Treasures are minted with their intrinsic sacrifice-for-mana
/// activation via the commodity-token path.
fn granted_attack_treasures(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let GameEvent::CreatureAttacks {
        defending: DefendingEntity::Player(defender), ..
    } = &trig.trigger_event else {
        // Attacking a planeswalker or battle, not a player: no effect.
        return Vec::new();
    };
    let defender_life = state.player(*defender).life;
    let none_richer = state
        .opponents_of(trig.controller)
        .all(|opp| state.player(opp).life <= defender_life);
    if !none_richer {
        return Vec::new();
    }
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Treasure,
        count: 2,
    }]
}
