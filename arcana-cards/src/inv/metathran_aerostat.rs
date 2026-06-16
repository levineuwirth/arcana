//! Metathran Aerostat — `{2}{U}{U}` 2/2 blue Metathran with Flying.
//! "{X}{U}: You may put a creature card with mana value X from your hand onto the
//! battlefield. If you do, return this creature to its owner's hand."

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
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
    let name = reg.interner_mut().intern("Metathran Aerostat");
    let metathran = reg.interner_mut().intern("Metathran");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(metathran);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{X}{U}: You may put a creature card with mana value X from your hand onto the battlefield. If you do, return this creature to its owner's hand.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{X}{U}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: put_creature,
            }),
    )
}

fn put_creature(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let x = ctx.x_value.unwrap_or(0);
    // GAP: "If you do, return this creature to its owner's hand" — the
    // conditional self-bounce (only when a card was actually put in) is not
    // expressible; the put is a may, so the bounce is omitted to avoid bouncing
    // when nothing was put.
    vec![Effect::PutFromHandOntoBattlefield {
        player: ctx.controller,
        filter: ObjectFilter::creature().with_min_cmc(x).with_max_cmc(x),
        tapped: false,
    }]
}
