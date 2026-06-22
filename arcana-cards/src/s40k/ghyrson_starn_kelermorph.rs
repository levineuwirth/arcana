//! Ghyrson Starn, Kelermorph — `{1}{U}{R}` 3/2 Legendary Creature —
//! Tyranid Human (R/U).
//!
//! * Ward {2} (parametrized keyword — fully implemented).
//! * Three Autostubs — "Whenever another source you control deals
//!   exactly 1 damage to a permanent or player, Ghyrson Starn deals 2
//!   damage to that permanent or player."
//!
//! GAP: Three Autostubs. The catalog's only damage-watching trigger is
//! `TriggerCondition::DamageDealt { source_filter, target_filter,
//! combat_only }`, which has NO amount predicate ("exactly 1") and no
//! accessor to recover the original damage's recipient as a
//! `DamageTarget` for the 2-damage payoff. Modeling it faithfully would
//! require an amount-gated trigger variant + a "damaged object/player"
//! accessor neither of which the demonstrated API exposes — and a
//! literal/approximation here would be a materially wrong card. Emitted
//! as a GAP'd trigger so the bones + Ward land.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ghyrson Starn, Kelermorph");
    let tyranid = reg.interner_mut().intern("Tyranid");
    let human = reg.interner_mut().intern("Human");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tyranid);
    subtypes.0.insert(human);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Ward(ManaCost::parse("{2}").expect("valid cost"))],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::permanent()
                        .controlled_by(ControllerConstraint::You),
                    target_filter: TargetFilter::AnyTarget,
                    combat_only: false,
                },
                intervening_if: None,
                effect: three_autostubs,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn three_autostubs(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "exactly 1 damage … deals 2 damage to THAT permanent or
    // player". No amount predicate on DamageDealt and no
    // damaged-recipient accessor to retarget the 2-damage payoff.
    Vec::new()
}
