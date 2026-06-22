//! Glarb, Calamity's Augur — `{B}{G}{U}` 2/4 Legendary Creature — Frog Wizard Noble.
//! Deathtouch.
//! You may look at the top card of your library any time. (GAP: static info ability.)
//! You may play lands and cast spells with mana value 4 or greater from the top
//! of your library. (GAP: top-of-library play permission static.)
//! `{T}: Surveil 2.`

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Glarb, Calamity's Augur");
    let frog = reg.interner_mut().intern("Frog");
    let wizard = reg.interner_mut().intern("Wizard");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(frog);
    subtypes.0.insert(wizard);
    subtypes.0.insert(noble);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{G}{U}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    // GAP: "You may look at the top card of your library any time" — static info ability.
    // GAP: "You may play lands and cast spells with mana value 4 or greater from the
    //       top of your library" — top-of-library play-permission static.
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Surveil 2.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: surveil_two,
            }),
    )
}

fn surveil_two(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Surveil { player: ctx.controller, count: 2 }]
}
