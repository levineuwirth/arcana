//! Alpharael, Stonechosen — `{3}{B}{B}` 3/3 Legendary Creature — Human Cleric.
//! Ward—Discard a card at random.
//! Void — Whenever Alpharael attacks, if a nonland permanent left the
//! battlefield this turn or a spell was warped this turn, defending player
//! loses half their life, rounded up.
//!
//! Ward with a non-mana cost ("Discard a card at random") is not expressible
//! as `KeywordAbility::Ward(ManaCost)`, so the keyword line is empty and the
//! ward is GAP'd. The Void attack trigger is wired; the defending player
//! loses half their life rounded up (computed at resolution). The
//! intervening-if gate ("if a nonland permanent left the battlefield this
//! turn or a spell was warped this turn") has no demonstrated condition
//! predicate and is GAP'd (trigger fires unconditionally).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Alpharael, Stonechosen");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: "Ward—Discard a card at random." is a non-mana ward cost,
        // not expressible as KeywordAbility::Ward(ManaCost).
        // GAP: "Void" ability-word marker (no mechanical effect on its own).
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            // GAP: intervening-if "if a nonland permanent left the battlefield
            // this turn or a spell was warped this turn" — no demonstrated
            // condition predicate for left-the-battlefield-this-turn / warped.
            intervening_if: None,
            effect: void_lose_half_life,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn void_lose_half_life(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(p) = trig.defending_player() else {
        return Vec::new();
    };
    let life = script::life(state, p).max(0) as u32;
    let half = (life + 1) / 2; // rounded up
    if half == 0 {
        return Vec::new();
    }
    vec![Effect::LoseLife {
        player: p,
        amount: half,
    }]
}
