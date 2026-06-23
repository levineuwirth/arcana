//! Mona Lisa, Science Geek — `{2}{G}` 1/3 Legendary Creature — Lizard Mutant.
//! Reach.
//! {T}: Add X mana of any one color, where X is Mona Lisa's power.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mona Lisa, Science Geek");
    let lizard = reg.interner_mut().intern("Lizard");
    let mutant = reg.interner_mut().intern("Mutant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lizard);
    subtypes.0.insert(mutant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}: Add X mana of any one color, where X is Mona Lisa's power.".into(),
            cost: ActivationCost::tap_only(),
            target_requirements: Vec::new(),
            is_mana_ability: true,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: add_x_mana,
        }),
    )
}

fn add_x_mana(state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // X is this creature's power.
    let x = script::power_of(state, ctx.source).max(0) as usize;
    // GAP (fidelity): "any one color" — the player's color choice isn't
    // modeled; produces X green mana (its printed color) as best-effort.
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Green, ctx.source); x],
    }]
}
