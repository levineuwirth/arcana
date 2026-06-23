//! Krile Baldesion — `{W}{U}` 2/1 Legendary Dwarf Wizard.
//!
//! Lifelink.
//! Trace Aether — Whenever you cast a noncreature spell, you may return
//! target creature card with mana value equal to that spell's mana value
//! from your graveyard to your hand. Do this only once each turn.

use arcana_core::actions::OptionalPaymentKind;
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

// GAP: "with mana value equal to that spell's mana value" — the target's mana
// value must equal the just-cast spell's mana value, a per-event dynamic
// constraint not expressible in a static ObjectFilter. The target is restricted
// to a creature card in your graveyard; the MV-equality is not enforced.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Krile Baldesion");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: Some(ObjectFilter::new().without_types(TypeLine::CREATURE.into())),
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: may_return_creature,
            trigger_zones: vec![Zone::Battlefield],
            // "Do this only once each turn."
            frequency: TriggerFrequency::OncePerTurn,
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

/// "You may return target creature card … to your hand." The optional ("you
/// may") is modeled as a free OptionalPayment whose payload returns the chosen
/// graveyard card.
fn may_return_creature(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Life(0),
        then: Box::new(Effect::ReturnFromGraveyardToHand { target: *id }),
        else_effect: None,
    }]
}
