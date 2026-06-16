//! Bruse Tarl, Roving Rancher — `{2}{R}{W}` Legendary 4/3 Human Warrior.
//!
//! Oracle:
//! * Oxen you control have double strike. (GAP: static continuous anthem —
//!   no trigger/cost, not expressible as a triggered/activated ability.)
//! * Whenever Bruse Tarl enters or attacks, exile the top card of your
//!   library. If it's a land card, create a 2/2 white Ox creature token.
//!   Otherwise, you may cast it until the end of your next turn. (Modeled
//!   as two triggers — SelfEntersBattlefield and SelfAttacks — but the
//!   effect itself is a GAP: the engine has no "exile top, branch on land
//!   vs. cast-until-next-turn" primitive. ImpulseExile is single-turn and
//!   lacks the land→token branch, so it would be materially wrong.)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bruse Tarl, Roving Rancher");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: "Oxen you control have double strike." — a static continuous
    // anthem with no trigger word or activation cost; not expressible as a
    // triggered or activated ability.

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: exile_top_and_branch,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: exile_top_and_branch,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Exile the top card of your library; if a land, make a 2/2 Ox, else you
/// may cast it until the end of your next turn.
fn exile_top_and_branch(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no primitive exiles the top card and branches on "is it a land?"
    // (land → create a 2/2 white Ox token; otherwise gain a cast-until-end-
    // of-your-next-turn permission). ImpulseExile is single-turn and has no
    // land→token branch, so emitting it would be materially wrong.
    Vec::new()
}
