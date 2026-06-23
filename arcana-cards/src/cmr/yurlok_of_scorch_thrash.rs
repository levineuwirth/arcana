//! Yurlok of Scorch Thrash — `{1}{B}{R}{G}` 4/4 Legendary Lizard Shaman
//! with Vigilance.
//!
//! * Vigilance — base keyword.
//! * GAP: "A player losing unspent mana causes that player to lose that
//!   much life." — a static replacement-style ability tied to emptying
//!   the mana pool; no replacement/empty-pool hook is in the
//!   demonstrated API, so it is GAP'd.
//! * "{1}, {T}: Each player adds {B}{R}{G}." — a mana-and-tap activated
//!   ability; resolution adds {B}{R}{G} to every player's pool via one
//!   `AddMana` per player. Modeled as a non-mana ability (it adds to
//!   multiple players, not only the controller).

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Yurlok of Scorch Thrash");
    let lizard = reg.interner_mut().intern("Lizard");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lizard);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{1}, {T}: Each player adds {B}{R}{G}.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: each_player_adds_brg,
        }),
    )
}

fn each_player_adds_brg(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    script::all_players(state)
        .into_iter()
        .map(|p| Effect::AddMana {
            player: p,
            mana: vec![
                ManaUnit::plain(ManaColor::Black, ctx.source),
                ManaUnit::plain(ManaColor::Red, ctx.source),
                ManaUnit::plain(ManaColor::Green, ctx.source),
            ],
        })
        .collect()
}
