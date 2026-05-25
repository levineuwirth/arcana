//! Totem-Guide Hartebeest — `{4}{W}` 2/5 white Antelope creature.
//! "When this creature enters, you may search your library for an
//! Aura card, reveal it, put it into your hand, then shuffle."
//!
//! Modeled as an ETB `TutorToHand` filtered to the Aura subtype.
//! GAP: the "you may" optionality is not expressible with the
//! catalog — the search resolves unconditionally.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Totem-Guide Hartebeest");
    let antelope = reg.interner_mut().intern("Antelope");
    // Pre-intern the Aura subtype so the resolver can build the
    // search filter via the non-mut interner at resolve time.
    let _aura = reg.interner_mut().intern("Aura");
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

/// ETB trigger resolution: search the controller's library for an
/// Aura card and put it into their hand (revealed, shuffle after).
fn etb_tutor_aura(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = script::subtype_filter(reg, "Aura");
    vec![Effect::TutorToHand {
        player: trig.controller,
        filter,
        reveal: true,
    }]
}
