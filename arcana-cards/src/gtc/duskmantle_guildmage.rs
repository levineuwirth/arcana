//! Duskmantle Guildmage — `{U}{B}` 2/2 Human Wizard.
//! "{1}{U}{B}: Whenever a card is put into an opponent's graveyard from
//! anywhere this turn, that player loses 1 life."
//! "{2}{U}{B}: Target player mills two cards."
//!
//! Two activated abilities. The first installs a floating, this-turn-only
//! triggered ability watching opponents' graveyard fills — that delayed
//! self-installing watcher is not expressible with the demonstrated API, so
//! its effect is GAP'd. The second ability is a straightforward mill.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Duskmantle Guildmage");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{U}{B}: Whenever a card is put into an opponent's graveyard from anywhere this turn, that player loses 1 life.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{U}{B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: install_grief_watcher,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{U}{B}: Target player mills two cards.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{U}{B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: mill_target_player,
            }),
    )
}

fn install_grief_watcher(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    Vec::new()
    // GAP: installs a this-turn-only floating triggered ability ("whenever a
    // card is put into an opponent's graveyard, that player loses 1 life") —
    // a self-installing delayed watcher is not expressible with the
    // demonstrated effect API.
}

fn mill_target_player(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Player(p) = target else {
        return Vec::new();
    };
    vec![Effect::Mill {
        player: *p,
        count: 2,
    }]
}
