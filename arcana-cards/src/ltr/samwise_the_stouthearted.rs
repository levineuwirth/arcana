//! Samwise the Stouthearted — `{1}{W}` 2/1 Legendary Creature — Halfling
//! Peasant with Flash.
//!
//! Oracle:
//! * Flash (base keyword).
//! * When Samwise enters, choose up to one target permanent card in your
//!   graveyard that was put there from the battlefield this turn. Return it to
//!   your hand. Then the Ring tempts you.
//!   - "Return it to your hand" is wired via ReturnFromGraveyardToHand.
//!   - FIDELITY GAP: the "put there from the battlefield this turn" target
//!     restriction is not expressible with the demonstrated ObjectFilter
//!     surface.
//!   - GAP: "the Ring tempts you" — the Ring-bearer / Ring mechanic has no
//!     demonstrated Effect primitive.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Samwise the Stouthearted");
    let halfling = reg.interner_mut().intern("Halfling");
    let peasant = reg.interner_mut().intern("Peasant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(halfling);
    subtypes.0.insert(peasant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: return_permanent_card,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::permanent().controlled_by(ControllerConstraint::You),
                },
                count: TargetCount::UpTo(1),
                controller: None,
            }],
        }),
    )
}

fn return_permanent_card(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        // GAP: "the Ring tempts you" — no demonstrated Ring/Ring-bearer
        // Effect primitive; still resolve as a no-op if no card chosen.
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // GAP: "Then the Ring tempts you" — Ring mechanic has no demonstrated
    // Effect primitive; only the return-to-hand portion is wired.
    vec![Effect::ReturnFromGraveyardToHand { target: *id }]
}
