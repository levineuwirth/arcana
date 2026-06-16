//! Tocasia, Dig Site Mentor — `{1}{G}{W}{U}` 4/3 Legendary Human Artificer.
//! Static anthem ("Creatures you control have vigilance and {T}: Surveil 1")
//! is a continuous ability GAP. The graveyard-activated reanimation
//! ("{2}{G}{G}{W}{W}{U}{U}, Exile this from your graveyard: return any
//! number of target artifact cards … from your graveyard to the
//! battlefield. Sorcery speed") is expressed via an exile-self graveyard
//! activation returning the chosen graveyard artifact cards.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tocasia, Dig Site Mentor");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{W}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: static "Creatures you control have vigilance and '{T}: Surveil 1.'"
    //      — a granted-keyword + granted-activated-ability anthem (continuous
    //      static, not a triggered/activated ability of this card).
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{G}{G}{W}{W}{U}{U}, Exile this card from your graveyard: \
                       Return any number of target artifact cards with total mana value \
                       10 or less from your graveyard to the battlefield. \
                       Activate only as a sorcery."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{G}{G}{W}{W}{U}{U}").expect("valid cost"),
                    exile_self: true,
                    ..ActivationCost::default()
                },
                // GAP (fidelity): "total mana value 10 or less" across the chosen
                //                 set is not expressible per-target; any number of
                //                 graveyard artifact cards may be chosen.
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::new().with_types(TypeLine::ARTIFACT.into()),
                    },
                    count: TargetCount::Any,
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Graveyard,
                is_instant_speed: false,
                face_gate: None,
                effect: reanimate_artifacts,
            }),
    )
}

fn reanimate_artifacts(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    ctx.targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => {
                Some(Effect::ReturnFromGraveyardToBattlefield { target: *id })
            }
            _ => None,
        })
        .collect()
}
