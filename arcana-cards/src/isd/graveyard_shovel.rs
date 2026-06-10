//! Graveyard Shovel — `{2}` artifact.
//! "{2}, {T}: Target player exiles a card from their graveyard. If it's a
//! creature card, you gain 2 life." The targeted player's own choice of a
//! graveyard card is modeled with `ChooseAnyNumberFromZone` (exile); the
//! exactly-one constraint and the conditional life gain are documented
//! gaps.

use arcana_core::effects::{Effect, PickAction};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Graveyard Shovel");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{2}, {T}: Target player exiles a card from their \
                       graveyard. If it's a creature card, you gain 2 life."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: exile_from_graveyard,
            },
        ),
    )
}

fn exile_from_graveyard(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: oracle is exactly ONE card; ChooseAnyNumberFromZone is a
    // min-0/max-all pick. GAP: 'If it's a creature card, you gain 2 life'
    // — the chosen card's type cannot be inspected after the pick.
    vec![Effect::ChooseAnyNumberFromZone {
        chooser: *p,
        zone: Zone::Graveyard(*p),
        filter: ObjectFilter::default(),
        action: PickAction::Exile,
    }]
}
