//! Ria Ivor, Bane of Bladehold — `{2}{W}{B}` 3/4 Legendary Phyrexian Knight.
//! Battle cry. "At the beginning of combat on your turn, the next time target
//! creature would deal combat damage to one or more players this combat,
//! prevent that damage. If damage is prevented this way, create that many 1/1
//! colorless Phyrexian Mite artifact creature tokens with toxic 1 and 'This
//! token can't block.'"

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::effects::Effect;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ria Ivor, Bane of Bladehold");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(knight);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::BattleCry],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // At the beginning of combat on your turn, target creature: prevent its
            // next combat damage to players, then create that-many Phyrexian Mites.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: prevent_then_mites,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn prevent_then_mites(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "the next time target creature would deal combat damage to one or more
    // players, prevent it; if prevented, create that-many Mites" is a one-shot
    // replacement-with-feedback effect (prevented amount → token count) not
    // expressible with the available prevention / token primitives.
    Vec::new()
}
