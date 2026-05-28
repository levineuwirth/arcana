//! Queen Kayla bin-Kroog — `{1}{R}{W}` 2/3 Legendary Human Noble.
//! `{4}, {T}: Discard all the cards in your hand, then draw that many cards. You may choose
//! an artifact or creature card with mana value 1 you discarded this way, then do the same
//! for mv 2 and 3. Return those cards to the battlefield. Activate only as a sorcery.`
//! GAP: "discard all cards in hand, draw that many" is dynamic (hand_size). GAP: "may
//! choose card discarded this way" requires tracking which cards were just discarded.
//! GAP: putting chosen discards onto battlefield requires targeted discard tracking.
//! Full effect not expressible; emitting Vec::new().

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::effects::Effect;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Queen Kayla bin-Kroog");
    let human = reg.interner_mut().intern("Human");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(noble);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
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
                text: "{4}, {T}: Discard all the cards in your hand, then draw that many cards. You may choose an artifact or creature card with mana value 1 you discarded this way, then do the same for artifact or creature cards with mana values 2 and 3. Return those cards to the battlefield. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}").unwrap(),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: hand_refuel,
            }),
    )
}

fn hand_refuel(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "discard all cards, draw that many" — dynamic hand_size; discard-all not a
    // single Effect variant. GAP: "you may choose discarded cards by mv and return to
    // battlefield" requires tracking which cards were just discarded — no engine support.
    Vec::new()
}
