//! Inga and Esika — `{2}{G}{U}` 4/4 Legendary Human God (green/blue).
//!
//! Oracle:
//! * Creatures you control have vigilance and "{T}: Add one mana of any
//!   color. Spend this mana only to cast a creature spell." (static grant)
//! * Whenever you cast a creature spell, if three or more mana from creatures
//!   was spent to cast it, draw a card.
//!
//! The first line is a pure static continuous ability granting an anthem
//! keyword + a granted mana ability to your creatures — not expressible as a
//! triggered/activated ability (GAP). The cast trigger uses `SpellCast`, but
//! its gate ("if three or more mana from creatures was spent") needs a
//! mana-source accessor that is not exposed; firing the draw on every creature
//! spell would be materially wrong, so the effect is GAP'd while the trigger
//! shape is recorded.

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
    let name = reg.interner_mut().intern("Inga and Esika");
    let human = reg.interner_mut().intern("Human");
    let god = reg.interner_mut().intern("God");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(god);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: "Creatures you control have vigilance and '{T}: Add one mana of any
    //   color. Spend this mana only to cast a creature spell.'" — pure static
    //   continuous keyword/ability grant to other creatures.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter::new().with_types(TypeLine::CREATURE.into())),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: cast_creature_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn cast_creature_draw(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if three or more mana from creatures was spent to cast it" — no
    // accessor for the source of mana spent; firing the draw unconditionally
    // on every creature spell would be materially wrong.
    Vec::new()
}
