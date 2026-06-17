//! The Gunky Runner — `{5}{B}` 5/5 Legendary Slug Gamer.
//!
//! Oracle:
//! * Ew-minance — ETB / first-upkeep-as-commander: shuffle three Gunk cards
//!   into each opponent's library. (Gunk-card minting / shuffle-into-library —
//!   GAP: no Effect mints a named card into an opponent's library.)
//! * Whenever an opponent loses life, they shuffle that many Gunk cards into
//!   their library. (GAP: no LifeLost trigger condition.)
//! * Ready to run — commander-format static, not modeled.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::effects::Effect;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Gunky Runner");
    let slug = reg.interner_mut().intern("Slug");
    let gamer = reg.interner_mut().intern("Gamer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(slug);
    subtypes.0.insert(gamer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        // GAP: "Ew-minance" / "Ready to run" — non-standard custom keywords
        // with no KeywordAbility variant; omitted.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // ETB half of Ew-minance fires; the "first upkeep as commander"
            // half is unmodeled and the Gunk-shuffle payload is unexpressible.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: shuffle_gunk,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
    // GAP: "Whenever an opponent loses life, they shuffle that many Gunk
    // cards into their library." — there is no LifeLost TriggerCondition.
}

fn shuffle_gunk(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "shuffle three Gunk cards into each opponent's library" — minting a
    // specific named card into an opponent's library has no Effect variant.
    Vec::new()
}
