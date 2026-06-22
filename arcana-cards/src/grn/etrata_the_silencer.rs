//! Etrata, the Silencer — `{2}{U}{B}` 3/5 Legendary Vampire Assassin.
//!
//! Oracle:
//! * "Etrata can't be blocked."
//! * "Whenever Etrata deals combat damage to a player, exile target creature
//!   that player controls and put a hit counter on that card. That player
//!   loses the game if they own three or more exiled cards with hit counters
//!   on them. Etrata's owner shuffles Etrata into their library."
//!
//! "Can't be blocked" is delivered as a self-applied CantBeBlocked on ETB for
//! the duration it remains on the battlefield (the only way to hang a static
//! evasion with the available surface). The combat-damage trigger body is
//! GAP'd: it must exile a creature controlled specifically by the DAMAGED
//! player (no such dynamic controller-restricted target filter), put a hit
//! counter on the exiled CARD (AddCounters targets battlefield permanents,
//! not exiled cards), enforce a three-hit-counter lose-the-game state, and
//! shuffle Etrata into its library — none of these are expressible.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Etrata, the Silencer");
    let vampire = reg.interner_mut().intern("Vampire");
    let assassin = reg.interner_mut().intern("Assassin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(assassin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: make_unblockable,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: combat_damage_exile_hit,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_unblockable(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::CantBeBlocked {
        target: trig.source,
        duration: Duration::WhileSourceOnBattlefield,
    }]
}

fn combat_damage_exile_hit(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: exile a creature controlled by the DAMAGED player (no dynamic
    // controller-restricted target filter), put a hit counter on the exiled
    // CARD (AddCounters is battlefield-scoped), enforce the three-hit lose
    // state, and shuffle Etrata into its library — none expressible.
    Vec::new()
}
