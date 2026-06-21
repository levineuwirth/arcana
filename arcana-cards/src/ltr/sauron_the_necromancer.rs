//! Sauron, the Necromancer — `{3}{B}{B}` 4/4 Legendary Creature — Avatar Horror.
//!
//! Oracle:
//! * Menace.
//! * Whenever Sauron attacks, exile target creature card from your graveyard.
//!   Create a tapped and attacking token that's a copy of that card, except
//!   it's a 3/3 black Wraith with menace. At the beginning of the next end
//!   step, exile that token unless Sauron is your Ring-bearer.
//!
//! We express the attack trigger that exiles a target creature card from
//! your graveyard. The "create a tapped-and-attacking token that's a copy of
//! that exiled card (overridden to a 3/3 Wraith)" portion is GAP'd: there is
//! no primitive that copies a card AS a tapped-and-attacking token with a
//! stat/type/keyword override, and the Ring-bearer end-step exile gate is
//! likewise unmodeled.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::effects::KeywordAbility;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sauron, the Necromancer");
    let avatar = reg.interner_mut().intern("Avatar");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(avatar);
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: exile_creature_card_from_graveyard,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                },
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn exile_creature_card_from_graveyard(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: "Create a tapped and attacking token that's a copy of that card,
    // except it's a 3/3 black Wraith with menace" — no primitive copies an
    // exiled card as a tapped/attacking token with a stat override.
    // GAP: "At the beginning of the next end step, exile that token unless
    // Sauron is your Ring-bearer" — Ring-bearer state is unmodeled.
    vec![Effect::ExileFromGraveyard { target: *id }]
}
