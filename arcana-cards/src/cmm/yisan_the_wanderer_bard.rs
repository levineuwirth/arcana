//! Yisan, the Wanderer Bard — `{2}{G}` 2/3 Legendary Human Rogue Bard.
//! `{2}{G}, {T}, Put a verse counter on Yisan: Search your library for a
//! creature card with mana value equal to the number of verse counters on
//! Yisan, put it onto the battlefield, then shuffle.`
//! GAP: Cannot filter by "mana value equal to the number of verse counters
//! on this permanent" — ObjectFilter has no dynamic-cmc-eq-counter field.
//! Falling back to searching for any creature card (unconstrained MV).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Yisan, the Wanderer Bard");
    let human = reg.interner_mut().intern("Human");
    let rogue = reg.interner_mut().intern("Rogue");
    let bard = reg.interner_mut().intern("Bard");
    let verse = reg.interner_mut().intern("verse");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rogue);
    subtypes.0.insert(bard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{G}, {T}, Put a verse counter on Yisan: Search your library for a creature card with mana value equal to the number of verse counters on Yisan, put it onto the battlefield, then shuffle.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{G}").unwrap(),
                    tap: true,
                    add_self_counter: Some((CounterKind::Named(verse), 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: tutor_creature_by_verse_count,
            }),
    )
}

fn tutor_creature_by_verse_count(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: cannot filter by "mana value equal to the number of verse counters
    // on this permanent" — ObjectFilter has no dynamic-cmc-eq-counter field.
    // Falling back to "search for any creature card" (unconstrained MV).
    vec![Effect::TutorToBattlefield {
        player: ctx.controller,
        filter: ObjectFilter::creature(),
        tapped: false,
    }]
}
