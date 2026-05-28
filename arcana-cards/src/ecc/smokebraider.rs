//! Smokebraider — `{1}{R}` 1/1 red Elemental Shaman.
//! "{T}: Add two mana in any combination of colors. Spend this mana only to cast Elemental spells
//! or activate abilities of Elementals."
//! GAP: "spend only on Elemental spells" — mana restriction not in Effect::AddMana.
//! GAP: "two mana in any combination of colors" — player choice of colors not modeled.
//! Using two red mana as placeholder.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Smokebraider");
    let elemental = reg.interner_mut().intern("Elemental");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(shaman);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add two mana in any combination of colors. Spend this mana only to cast Elemental spells or activate abilities of Elementals.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                // GAP: mana ability but has restriction — treating as is_mana_ability true
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_two_mana,
            }),
    )
}

fn add_two_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "two mana in any combination of colors" — using two red as placeholder.
    // GAP: "spend only on Elemental spells" — restriction not modeled.
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![
            ManaUnit::plain(ManaColor::Red, ctx.source),
            ManaUnit::plain(ManaColor::Red, ctx.source),
        ],
    }]
}
