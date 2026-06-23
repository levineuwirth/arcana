//! Narset, Enlightened Exile — `{1}{U}{R}{W}` 3/4 Legendary Human Monk.
//!
//! Oracle:
//! * Creatures you control have prowess. (Static keyword-granting continuous
//!   ability — WIRED via a SelfEntersBattlefield trigger that installs a
//!   controller-wide `keyword_anthem(Prowess)` lasting while Narset is on the
//!   battlefield. See `po2/glorious_anthem.rs` for the install idiom.)
//! * Whenever Narset attacks, exile target noncreature, nonland card with mana
//!   value less than Narset's power from a graveyard and copy it. You may cast
//!   the copy without paying its mana cost.
//!   GAP: no combined exile-from-graveyard-then-copy-then-cast-for-free effect,
//!   and the "mana value less than Narset's power" dynamic target restriction
//!   is not expressible.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Narset, Enlightened Exile");
    let human = reg.interner_mut().intern("Human");
    let monk = reg.interner_mut().intern("Monk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(monk);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}{W}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: "Whenever Narset attacks, exile target noncreature/nonland card of
    //      mana value < Narset's power from a graveyard, copy it, and cast the
    //      copy for free." No exile-from-graveyard-then-copy-then-cast-free
    //      effect, and the dynamic-mv target restriction isn't expressible.
    reg.register(
        CardDefinition::new(name, chars)
            // "Creatures you control have prowess." — install a controller-wide
            // keyword anthem when Narset enters; it auto-expires when she leaves.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_prowess_anthem,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB: install "creatures you control have prowess" anchored to Narset,
/// lasting until she leaves the battlefield.
fn install_prowess_anthem(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::keyword_anthem(
            trig.source,
            trig.controller,
            KeywordAbility::Prowess,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
