//! Demilich — `{U}{U}{U}{U}` 4/3 Skeleton Wizard.
//! This spell costs {U} less for each instant/sorcery spell you've cast
//! this turn. (GAP'd — static cost reduction.)
//! Whenever this creature attacks, exile up to one target instant or
//! sorcery card from your graveyard. Copy it. You may cast the copy.
//! (the exile is modeled; "copy it / you may cast the copy" has no
//! copy-a-graveyard-card-and-cast primitive — fidelity gap.)
//! You may cast this card from your graveyard by exiling four instant
//! and/or sorcery cards from your graveyard... (GAP'd — alternative
//! cast-from-graveyard permission, no expressible hook.)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Demilich");
    let skeleton = reg.interner_mut().intern("Skeleton");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(skeleton);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{U}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: attack_exile_isorc,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::new().with_types_any(TypeLine(
                        TypeLine::INSTANT | TypeLine::SORCERY,
                    )),
                },
                count: TargetCount::UpTo(1),
                controller: None,
            }],
        }),
    )
}

fn attack_exile_isorc(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // "Copy it. You may cast the copy." — no primitive copies a card in a
    // graveyard and casts the copy. Modeled as exile only (fidelity gap).
    vec![Effect::ExileFromGraveyard { target: *id }]
}
