//! Baru, Wurmspeaker — `{2}{G}{G}` Legendary 3/3 Human Druid.
//! Wurms you control get +2/+2 and have trample.
//! {7}{G}, {T}: Create a 4/4 green Wurm creature token. This ability costs
//! {X} less to activate, where X is the greatest power among Wurms you
//! control.
//!
//! GAP: the static anthem "Wurms you control get +2/+2 and have trample" is
//! a continuous team buff with no demonstrated static primitive — omitted.
//! GAP: the activation cost reduction "costs {X} less … where X is the
//! greatest power among Wurms you control" is not expressible — the
//! activated ability is wired at its base cost `{7}{G}, {T}`.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Baru, Wurmspeaker");
    let human = reg.interner_mut().intern("Human");
    let druid = reg.interner_mut().intern("Druid");
    let _wurm = reg.interner_mut().intern("Wurm");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{7}{G}, {T}: Create a 4/4 green Wurm creature token. This \
                   ability costs {X} less to activate, where X is the greatest \
                   power among Wurms you control."
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{7}{G}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: make_wurm,
        }),
    )
}

fn make_wurm(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let wurm = reg.interner().lookup("Wurm").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wurm);
    let token = TokenDefinition {
        name: wurm,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![] as Vec<KeywordAbility>,
        abilities: vec![],
    };
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token,
    }]
}
