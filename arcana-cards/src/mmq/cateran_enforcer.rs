//! Cateran Enforcer — `{3}{B}{B}` 4/3 Horror Mercenary. Fear.
//! `{4}, {T}: Search your library for a Mercenary permanent card with
//! mana value 4 or less, put it onto the battlefield, then shuffle.`

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cateran Enforcer");
    let horror = reg.interner_mut().intern("Horror");
    let mercenary = reg.interner_mut().intern("Mercenary");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horror);
    subtypes.0.insert(mercenary);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Fear],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{4}, {T}: Search your library for a Mercenary permanent card with mana value 4 or less, put it onto the battlefield, then shuffle.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{4}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: tutor_mercenary,
        }),
    )
}

fn tutor_mercenary(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let merc = reg.interner().lookup("Mercenary").unwrap_or_default();
    vec![Effect::TutorToBattlefield {
        player: ctx.controller,
        filter: ObjectFilter::permanent().with_subtype_sym(merc).with_max_cmc(4),
        tapped: false,
    }]
}
