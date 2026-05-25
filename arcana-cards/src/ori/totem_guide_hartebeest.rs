//! Totem-Guide Hartebeest — `{4}{W}` 2/5 white creature. "When this
//! creature enters, you may search your library for an Aura card, reveal
//! it, put it into your hand, then shuffle."
//!
//! GAP: filter — ObjectFilter has no "subtype equals Aura" filter.
//! Using enchantment filter as best approximation.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Totem-Guide Hartebeest");
    let antelope = reg.interner_mut().intern("Antelope");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(antelope);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_tutor_aura,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_tutor_aura(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: filter — no Aura subtype filter; using enchantment type as approximation
    vec![Effect::TutorToHand {
        player: trig.controller,
        filter: ObjectFilter::permanent().with_types(TypeLine::ENCHANTMENT.into()),
        reveal: true,
    }]
}
