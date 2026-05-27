//! Boonweaver Giant — `{6}{W}` 4/4 white Giant Monk.
//! "When this creature enters, you may search your graveyard, hand, and/or library for
//! an Aura card and put it onto the battlefield attached to this creature. If you search
//! your library this way, shuffle."
//! GAP: TutorToBattlefield + Attach from graveyard/hand/library simultaneously is not
//! expressible as a single Effect variant; modeled as TutorToBattlefield for the library
//! search path only, Attach is omitted.

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
    let name = reg.interner_mut().intern("Boonweaver Giant");
    let giant = reg.interner_mut().intern("Giant");
    let monk = reg.interner_mut().intern("Monk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant);
    subtypes.0.insert(monk);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
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
    // GAP: Searching graveyard/hand is not modeled; TutorToBattlefield only covers library.
    // Attach to self after finding is also not modeled atomically.
    vec![Effect::TutorToBattlefield {
        player: trig.controller,
        filter: ObjectFilter::new().with_types(TypeLine::ENCHANTMENT.into()),
        tapped: false,
    }]
}
