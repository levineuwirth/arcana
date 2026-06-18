//! Renowned Weaponsmith — `{1}{U}` 1/3 Human Artificer.
//! `{T}: Add {C}{C}.` (GAP: the "spend only on artifact spells/abilities"
//! restriction is not expressible — the mana is added unrestricted.)
//! `{U}, {T}: Search your library for a card named Heart-Piercer Bow or
//! Vial of Dragonfire, reveal it, put it into your hand, then shuffle.`
//! GAP: tutor-by-name takes a single name; only Heart-Piercer Bow is
//! searched here (the OR of two named cards is not expressible).

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Renowned Weaponsmith");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {C}{C}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_two_colorless,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{U}, {T}: Search your library for a card named \
                       Heart-Piercer Bow or Vial of Dragonfire, reveal it, \
                       put it into your hand, then shuffle."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{U}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: tutor_named_weapon,
            }),
    )
}

fn add_two_colorless(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source); 2],
    }]
}

fn tutor_named_weapon(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let nm = reg.interner().lookup("Heart-Piercer Bow");
    vec![Effect::TutorToHand {
        player: ctx.controller,
        filter: ObjectFilter {
            name: nm,
            ..ObjectFilter::default()
        },
        reveal: true,
    }]
}
