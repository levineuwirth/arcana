//! Menagerie Curator — `{1}{G}` 1/3 Human Citizen.
//! "{T}: Add one mana of any one color. Spend this mana only to cast a
//! creature spell."
//! "Whenever you cast a creature spell that doesn't share a creature
//! type with a creature card in your library, draw a card."
//!
//! The mana ability ("any one color" + spend restriction) and the
//! library-type-comparison gate on the draw trigger are both
//! inexpressible with the demonstrated API, so each is GAP'd. The
//! SpellCast-of-a-creature trigger shape is kept; its conditional body
//! is GAP'd to avoid over-firing (it would otherwise draw on EVERY
//! creature spell).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Menagerie Curator");
    let human = reg.interner_mut().intern("Human");
    let citizen = reg.interner_mut().intern("Citizen");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(citizen);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: "{T}: Add one mana of any one color. Spend this mana only to cast a
    // creature spell." — AddMana requires a concrete color and there is no
    // restricted-spend mana primitive; the any-color choice + creature-only
    // restriction is inexpressible, so the whole mana ability is omitted.

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: Some(ObjectFilter::new().with_types(TypeLine::CREATURE.into())),
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: draw_if_unique_type,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn draw_if_unique_type(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "that doesn't share a creature type with a creature card in your
    // library" — comparing the cast spell's creature types against every
    // creature card in the controller's library is not expressible with the
    // script:: helpers; firing unconditionally would draw on every creature
    // spell (materially wrong), so the body is GAP'd.
    Vec::new()
}
