//! Bilbo, Birthday Celebrant — `{W}{B}{G}` 2/3 Legendary Halfling Rogue.
//! If you would gain life, you gain that much life plus 1 instead.
//! `{2}{W}{B}{G}, {T}, Exile Bilbo: Search your library for any number of
//! creature cards, put them onto the battlefield, then shuffle. Activate only
//! if you have 111 or more life.`

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{
    CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bilbo, Birthday Celebrant");
    let halfling = reg.interner_mut().intern("Halfling");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(halfling);
    subtypes.0.insert(rogue);

    // GAP (static replacement): "If you would gain life, you gain that much life
    // plus 1 instead." — a life-gain amount-modifying replacement effect; there
    // is no Effect primitive that installs an additive life-gain replacement, so
    // this static is omitted.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}{W}{B}{G}, {T}, Exile Bilbo: Search your library for any number of creature cards, put them onto the battlefield, then shuffle. Activate only if you have 111 or more life.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}{W}{B}{G}").expect("valid cost"),
                tap: true,
                exile_self: true,
                activation_condition: Some(if_life_111),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: search_creatures_to_battlefield,
        }),
    )
}

fn if_life_111(s: &GameState, _src: ObjectId, you: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::life_at_least(s, you, 111)
}

fn search_creatures_to_battlefield(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Partial: oracle searches for ANY NUMBER of creature cards; TutorToBattlefield
    // tutors a single creature card (no variable-count search-to-battlefield), so
    // this resolves a single creature onto the battlefield. The shuffle is automatic.
    vec![Effect::TutorToBattlefield {
        player: ctx.controller,
        filter: ObjectFilter::creature(),
        tapped: false,
    }]
}
