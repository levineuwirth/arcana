//! Fabrication Foundry — `{1}{W}` white artifact (The Brothers' War,
//! 2022). "{T}: Add {W}. Spend this mana only to cast an artifact spell or
//! activate an ability of an artifact source." and "{2}{W}, {T}, Exile one
//! or more other artifacts you control with total mana value X: Return
//! target artifact card with mana value X or less from your graveyard to
//! the battlefield. Activate only as a sorcery."

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fabrication Foundry");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                // GAP: "Spend this mana only to cast an artifact spell or
                // activate an ability of an artifact source" — mana spend
                // restrictions are not expressible; plain {W} is added.
                text: "{T}: Add {W}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_white_mana,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{W}, {T}, Exile one or more other artifacts you \
                       control with total mana value X: Return target \
                       artifact card with mana value X or less from your \
                       graveyard to the battlefield. Activate only as a \
                       sorcery."
                    .into(),
                // GAP: "Exile one or more other artifacts you control with
                // total mana value X" — an exile-permanents activation
                // cost is not expressible (only the mana + tap parts are
                // paid), and the X-to-target mana-value link cannot be
                // enforced.
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{W}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::new()
                            .with_types(TypeLine::ARTIFACT.into()),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: reanimate_artifact,
            }),
    )
}

fn add_white_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::White, ctx.source)],
    }]
}

fn reanimate_artifact(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::ReturnFromGraveyardToBattlefield { target: *id }]
}
