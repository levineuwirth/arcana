//! Marrow-Gnawer — `{3}{B}{B}` 2/3 Legendary Rat Rogue.
//! All Rats have fear (static — GAP).
//! {T}, Sacrifice a Rat: Create X 1/1 black Rat creature tokens, where
//! X is the number of Rats you control.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Marrow-Gnawer");
    let rat = reg.interner_mut().intern("Rat");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rat);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    let rat_filter = ObjectFilter::creature().with_subtype_sym(rat);

    // GAP: "All Rats have fear." is a static board-wide keyword grant,
    // not a triggered/activated ability.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}, Sacrifice a Rat: Create X 1/1 black Rat creature tokens, where X is the number of Rats you control.".into(),
            cost: ActivationCost {
                tap: true,
                sacrifice_other: Some(rat_filter),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: make_rat_tokens,
        }),
    )
}

fn make_rat_tokens(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let rat = match reg.interner().lookup("Rat") {
        Some(r) => r,
        None => return Vec::new(),
    };
    let filter = ObjectFilter::creature().with_subtype_sym(rat);
    let n = script::count_matching(state, &filter, ctx.controller);
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rat);
    let token = TokenDefinition {
        name: rat,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    (0..n)
        .map(|_| Effect::CreateToken {
            controller: ctx.controller,
            token: token.clone(),
        })
        .collect()
}
