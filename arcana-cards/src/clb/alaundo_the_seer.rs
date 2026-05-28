//! Alaundo the Seer — `{2}{G}{U}` 3/5 Legendary Human Shaman.
//! `{T}: Draw a card, then exile a card from your hand and put time counters on it equal to
//! its mana value, with special cast-from-exile trigger when counters are removed.`
//! GAP: "exile card with time counters, cast from exile when last counter removed" —
//! no Effect for exile-with-time-counters or cast-from-exile mechanic.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Alaundo the Seer");
    let human = reg.interner_mut().intern("Human");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(shaman);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Draw a card, then exile a card from your hand and put time counters on it equal to its mana value. ...".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: alaundo_ability,
            }),
    )
}

fn alaundo_ability(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "exile card with time counters equal to mana value; cast from exile when last removed" —
    // no Effect for exile-with-time-counters or triggered cast-from-exile.
    // Only drawing the card is expressible.
    vec![Effect::DrawCards {
        player: ctx.controller,
        count: 1,
    }]
}
