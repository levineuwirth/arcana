//! Kami of Terrible Secrets — `{3}{B}` 3/4 black Spirit creature.
//! "When this creature enters, if you control an artifact and an
//! enchantment, you draw a card and you gain 1 life."
//!
//! Intervening-if is modeled via `conditions::you_control_a` on
//! `intervening_if`. The full clause is "artifact and an enchantment"; the
//! engine condition gates on controlling an artifact (the artifact half), and
//! the resolution-time effect is unconditional thereafter.

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::objects::ObjectId;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kami of Terrible Secrets");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                // Intervening-if via conditions::you_control_a (artifact half of the clause).
                intervening_if: Some(iif_control_artifact_and_enchantment),
                effect: etb_draw_and_gain,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn iif_control_artifact_and_enchantment(state: &GameState, _source: ObjectId, you: PlayerId) -> bool {
    conditions::you_control_a(state, you, &ObjectFilter::new().with_types(TypeLine::ARTIFACT.into()))
        && conditions::you_control_a(state, you, &ObjectFilter::new().with_types(TypeLine::ENCHANTMENT.into()))
}

/// ETB trigger resolution: controller draws one card and gains one life.
fn etb_draw_and_gain(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::DrawCards { player: trig.controller, count: 1 },
        Effect::GainLife { player: trig.controller, amount: 1 },
    ]
}
