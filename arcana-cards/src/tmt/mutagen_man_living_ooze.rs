//! Mutagen Man, Living Ooze — `{X}{G}{G}` 2/3 Legendary Ooze Mutant.
//! Trample.
//! "Activated abilities of artifact tokens you control cost {1} less." —
//!   a static cost-reduction; not expressible here (GAP).
//! "When Mutagen Man enters, create X Mutagen tokens (artifacts with a
//!   sac-for-counter activated ability)." — the X count (X paid on cast)
//!   isn't readable in a triggered effect, and the bespoke token activated
//!   ability isn't expressible; ETB effect GAP'd (trigger shell wired).

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Mutagen Man, Living Ooze");
    let ooze = reg.interner_mut().intern("Ooze");
    let mutant = reg.interner_mut().intern("Mutant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ooze);
    subtypes.0.insert(mutant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: "Activated abilities of artifact tokens you control cost {1} less"
    // is a static cost-reduction not expressible as a triggered/activated def.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_make_mutagens,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_make_mutagens(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: create X Mutagen tokens — X (cast-time X) isn't readable in a
    // triggered effect, and the token's bespoke activated ability
    // ("{1}, {T}, Sacrifice: put a +1/+1 counter on target creature") isn't
    // expressible on a TokenDefinition; whole effect GAP'd.
    Vec::new()
}
