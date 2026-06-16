//! The Gitrog, Ravenous Ride — `{3}{B}{G}` 6/5 Legendary Frog Horror Mount.
//! Trample, Haste; Saddle 1.
//! "Whenever The Gitrog deals combat damage to a player, you may sacrifice a
//!  creature that saddled it this turn. If you do, draw X cards, then put up to
//!  X land cards from your hand onto the battlefield tapped, where X is the
//!  sacrificed creature's power."
//!
//! Trample + Haste are base keywords. Saddle (a Mount keyword) is not in the
//! usable keyword surface and is GAP'd. The combat-damage trigger requires
//! "a creature that saddled it this turn" (no saddled-this-turn accessor) plus a
//! chained dynamic-X draw-then-put using the sacrificed creature's power — not
//! expressible, so the trigger shell is emitted with a GAP'd empty body.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("The Gitrog, Ravenous Ride");
    let frog = reg.interner_mut().intern("Frog");
    let horror = reg.interner_mut().intern("Horror");
    let mount = reg.interner_mut().intern("Mount");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(frog);
    subtypes.0.insert(horror);
    subtypes.0.insert(mount);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Trample, KeywordAbility::Haste],
        ..Default::default()
    };

    // GAP: Saddle 1 (Mount keyword) is not in the usable keyword surface.
    reg.register(
        CardDefinition::new(name, chars)
            // GAP: combat-damage payoff (sacrifice a creature that saddled it this turn,
            // draw X, put up to X lands from hand tapped where X = its power) — no
            // saddled-this-turn accessor and no chained dynamic-X draw-then-put primitive.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::default(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: gitrog_payoff,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn gitrog_payoff(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    Vec::new()
}
